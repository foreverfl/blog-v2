# =============================================================================
# Provider Settings
# =============================================================================

variable "aws_region" {
  description = "AWS region"
  type        = string
  default     = "ap-northeast-1"
}

# =============================================================================
# Network Module Variables
# =============================================================================

variable "vpc_id" {
  description = "Specific VPC ID to query. Empty = query all VPCs."
  type        = string
  default     = ""
}

# =============================================================================
# Security Module Variables (비활성)
# =============================================================================
# variable "allowed_ssh_cidrs" {
#   description = "CIDR blocks allowed for SSH access"
#   type        = list(string)
#   default     = []
# }

# =============================================================================
# Compute Module Variables (비활성)
# =============================================================================
# variable "include_stopped_instances" {
#   description = "Include stopped instances in query results"
#   type        = bool
#   default     = true
# }

# =============================================================================
# Observability Module Variables (비활성)
# =============================================================================
# variable "log_group_prefix" {
#   description = "Prefix to filter CloudWatch log groups"
#   type        = string
#   default     = ""
# }