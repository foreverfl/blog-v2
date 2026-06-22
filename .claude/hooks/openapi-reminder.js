#!/usr/bin/env node
// PostToolUse hook for blog-v2.
//
// When a route/handler source file is edited, nudge the model to update the
// matching OpenAPI spec. Anything else: stay silent.
//
// Claude Code pipes the tool event to us as JSON on stdin, and reads our stdout
// back: printing { hookSpecificOutput: { additionalContext } } injects that text
// into the model's context.

const fs = require('fs');

// 1. Read the event from stdin and pull out the edited file path.
let event;
try {
  event = JSON.parse(fs.readFileSync(0, 'utf8'));
} catch {
  process.exit(0); // no / unparseable input — nothing to do
}
const filePath = event?.tool_input?.file_path ?? '';
if (!filePath) process.exit(0);

// 2. Is it a route/handler file we care about?
const ROUTE_FILE_PATTERNS = [
  /\/services\/[^/]+\/src\/(routes|handlers)\/.+\.rs$/, // rust routes & handlers
  /\/services\/go\/.*(main|handler|router).*\.go$/, // go entrypoint / handlers
];
const isRouteFile = ROUTE_FILE_PATTERNS.some((pattern) => pattern.test(filePath));
if (!isRouteFile) process.exit(0);

// 3. Emit the reminder into the model's context.
const reminder =
  'Edited a route/handler in blog-v2. Per CLAUDE.md, update the matching OpenAPI ' +
  'spec under doc-source/openapi/specs/<service>/<domain>.yaml. ' +
  'The /add-openapi skill can regenerate the domain spec.';

console.log(
  JSON.stringify({
    hookSpecificOutput: {
      hookEventName: 'PostToolUse',
      additionalContext: reminder,
    },
  }),
);
