# =============================================================================
# Security Module Variables
# =============================================================================
# Security Groups, IAM Roles/Policies
# =============================================================================

variable "vpc_id" {
  description = "VPC ID to query security groups"
  type        = string
}

# =============================================================================
# Security Group Filters (for query)
# =============================================================================

variable "sg_name_filter" {
  description = "Filter security groups by name pattern (optional)"
  type        = string
  default     = "*"
}

# =============================================================================
# IAM Role Filters (for query)
# =============================================================================

variable "iam_role_name_prefix" {
  description = "Prefix to filter IAM roles (optional)"
  type        = string
  default     = ""
}

# =============================================================================
# Resource Creation Variables (for future use)
# =============================================================================

variable "create_default_sgs" {
  description = "Create default security groups (web, ssh, db)"
  type        = bool
  default     = false
}

variable "allowed_ssh_cidrs" {
  description = "CIDR blocks allowed for SSH access"
  type        = list(string)
  default     = []
}

variable "allowed_web_cidrs" {
  description = "CIDR blocks allowed for web access (80, 443)"
  type        = list(string)
  default     = ["0.0.0.0/0"]
}