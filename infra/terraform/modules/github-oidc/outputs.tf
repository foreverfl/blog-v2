# =============================================================================
# GitHub OIDC Module Outputs
# =============================================================================

output "role_arn" {
  description = "Role ARN for aws-actions/configure-aws-credentials role-to-assume"
  value       = aws_iam_role.github_actions_readonly.arn
}
