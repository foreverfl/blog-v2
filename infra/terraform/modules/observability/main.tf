# =============================================================================
# Observability Module - Resource Creation
# =============================================================================
# Currently query-only (data sources only)
# CloudWatch Alarms, Log Groups will be added here
# =============================================================================

# TODO: CloudWatch Log Group
# resource "aws_cloudwatch_log_group" "app" {
#   name              = "/blog-v2/application"
#   retention_in_days = 14
#
#   tags = {
#     Name = "blog-v2-app-logs"
#   }
# }

# TODO: CPU Utilization Alarm
# resource "aws_cloudwatch_metric_alarm" "cpu_high" {
#   for_each = toset(var.ec2_instance_ids)
#
#   alarm_name          = "cpu-high-${each.value}"
#   comparison_operator = "GreaterThanThreshold"
#   evaluation_periods  = 2
#   metric_name         = "CPUUtilization"
#   namespace           = "AWS/EC2"
#   period              = 300
#   statistic           = "Average"
#   threshold           = var.cpu_threshold
#   alarm_description   = "CPU utilization is above ${var.cpu_threshold}%"
#   alarm_actions       = var.alarm_actions
#
#   dimensions = {
#     InstanceId = each.value
#   }
#
#   tags = {
#     Name = "cpu-high-${each.value}"
#   }
# }

# TODO: Instance Status Check Alarm
# resource "aws_cloudwatch_metric_alarm" "status_check" {
#   for_each = toset(var.ec2_instance_ids)
#
#   alarm_name          = "status-check-${each.value}"
#   comparison_operator = "GreaterThanThreshold"
#   evaluation_periods  = 2
#   metric_name         = "StatusCheckFailed"
#   namespace           = "AWS/EC2"
#   period              = var.status_check_period
#   statistic           = "Maximum"
#   threshold           = 0
#   alarm_description   = "Instance status check failed"
#   alarm_actions       = var.alarm_actions
#
#   dimensions = {
#     InstanceId = each.value
#   }
#
#   tags = {
#     Name = "status-check-${each.value}"
#   }
# }