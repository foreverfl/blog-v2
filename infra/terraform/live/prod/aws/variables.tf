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
  description = "Target VPC ID (optional - uses first VPC if not specified)"
  type        = string
  default     = ""
}

variable "enable_nat" {
  description = "Enable NAT Gateway for private subnets"
  type        = bool
  default     = false
}

# =============================================================================
# Security Module Variables
# =============================================================================

variable "allowed_ssh_cidrs" {
  description = "CIDR blocks allowed for SSH access"
  type        = list(string)
  default     = []
}

# =============================================================================
# Compute Module Variables
# =============================================================================

variable "include_stopped_instances" {
  description = "Include stopped instances in query results"
  type        = bool
  default     = true
}

# =============================================================================
# Observability Module Variables
# =============================================================================

variable "log_group_prefix" {
  description = "Prefix to filter CloudWatch log groups"
  type        = string
  default     = ""
}