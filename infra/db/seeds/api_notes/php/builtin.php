<?php

$lines = [];
foreach (get_defined_functions()['internal'] as $function) {
    $extension = strtolower((new ReflectionFunction($function))->getExtensionName());
    $lines[] = $extension . "\t" . $function;
}
sort($lines, SORT_STRING);
echo implode("\n", $lines), "\n";
