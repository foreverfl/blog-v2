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

variable "nat_enabled" {
  description = "Create a NAT Gateway for private subnet internet access"
  type        = bool
  default     = false
}

variable "nat_subnet_id" {
  description = "Subnet ID to place the NAT Gateway in (must have IGW route)"
  type        = string
  default     = ""
}

variable "nat_private_route_table_ids" {
  description = "Route table IDs of private subnets to add NAT route to"
  type        = list(string)
  default     = []
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