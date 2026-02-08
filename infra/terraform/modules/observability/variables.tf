# =============================================================================
# Observability Module Variables
# =============================================================================
# CloudWatch Logs, Metrics, Alarms
# =============================================================================

variable "log_group_prefix" {
  description = "Prefix to filter CloudWatch log groups"
  type        = string
  default     = ""
}

variable "alarm_name_prefix" {
  description = "Prefix to filter CloudWatch alarms"
  type        = string
  default     = ""
}

# =============================================================================
# EC2 Instance IDs (alarm targets)
# =============================================================================

variable "ec2_instance_ids" {
  description = "EC2 instance IDs to monitor"
  type        = list(string)
  default     = []
}

# =============================================================================
# Alarm Creation Variables (for future use)
# =============================================================================

variable "create_default_alarms" {
  description = "Create default CloudWatch alarms"
  type        = bool
  default     = false
}

variable "alarm_actions" {
  description = "SNS topic ARNs for alarm notifications"
  type        = list(string)
  default     = []
}

variable "cpu_threshold" {
  description = "CPU utilization threshold for alarms (%)"
  type        = number
  default     = 80
}

variable "status_check_period" {
  description = "Period for status check alarms (seconds)"
  type        = number
  default     = 300
}