# =============================================================================
# CloudWatch Log Groups
# =============================================================================

data "aws_cloudwatch_log_groups" "all" {
  log_group_name_prefix = var.log_group_prefix != "" ? var.log_group_prefix : null
}

# =============================================================================
# Note: CloudWatch Alarms & SNS Topics
# =============================================================================
# aws_cloudwatch_metric_alarms, aws_sns_topics data sources don't exist.
# Use AWS CLI or Console to view:
#   aws cloudwatch describe-alarms
#   aws sns list-topics
# =============================================================================

# =============================================================================
# CloudWatch Dashboards
# =============================================================================

# 참고: aws_cloudwatch_dashboards data source는 없음
# Dashboard 목록은 AWS CLI로 확인: aws cloudwatch list-dashboards

# =============================================================================
# SSM Parameter Store (설정값 저장용)
# =============================================================================

data "aws_ssm_parameters_by_path" "config" {
  path            = "/blog-v2/"
  recursive       = true
  with_decryption = false
}

# =============================================================================
# Current Region & Account (참조용)
# =============================================================================

data "aws_region" "current" {}
data "aws_caller_identity" "current" {}