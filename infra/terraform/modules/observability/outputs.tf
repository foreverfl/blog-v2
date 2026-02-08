# =============================================================================
# CloudWatch Log Groups
# =============================================================================

output "log_groups" {
  description = "CloudWatch Log Groups"
  value = {
    count = length(data.aws_cloudwatch_log_groups.all.log_group_names)
    names = data.aws_cloudwatch_log_groups.all.log_group_names
    arns  = data.aws_cloudwatch_log_groups.all.arns
  }
}

# =============================================================================
# Note: CloudWatch Alarms & SNS Topics
# =============================================================================
# These data sources are not available in AWS provider.
# Use AWS CLI: aws cloudwatch describe-alarms / aws sns list-topics
# =============================================================================

# =============================================================================
# SSM Parameters
# =============================================================================

output "ssm_parameters" {
  description = "SSM Parameter Store entries"
  value = {
    count = length(data.aws_ssm_parameters_by_path.config.names)
    names = data.aws_ssm_parameters_by_path.config.names
  }
}

# =============================================================================
# Account Information
# =============================================================================

output "account_info" {
  description = "Current AWS account and region info"
  value = {
    account_id = data.aws_caller_identity.current.account_id
    region     = data.aws_region.current.name
    arn        = data.aws_caller_identity.current.arn
  }
}

# =============================================================================
# Observability Overview (한눈에 보기)
# =============================================================================

output "observability_overview" {
  description = "Overall observability status"
  value = {
    log_groups_count = length(data.aws_cloudwatch_log_groups.all.log_group_names)
    ssm_params_count = length(data.aws_ssm_parameters_by_path.config.names)
    region           = data.aws_region.current.name
  }
}

# =============================================================================
# CloudWatch Insights Query Examples (참고용)
# =============================================================================

output "useful_log_insights_queries" {
  description = "Useful CloudWatch Logs Insights query examples"
  value = {
    error_logs   = "fields @timestamp, @message | filter @message like /ERROR/ | sort @timestamp desc | limit 50"
    latency_p99  = "stats pct(@duration, 99) as p99 by bin(5m)"
    top_requests = "stats count() as cnt by @message | sort cnt desc | limit 10"
  }
}