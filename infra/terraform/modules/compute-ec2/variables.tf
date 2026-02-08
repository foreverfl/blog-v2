# =============================================================================
# Compute-EC2 Module Variables
# =============================================================================
# EC2 Instances query and management
# =============================================================================

variable "vpc_id" {
  description = "VPC ID to filter EC2 instances"
  type        = string
}

# =============================================================================
# Instance Filters (for query)
# =============================================================================

variable "instance_name_filter" {
  description = "Filter instances by Name tag pattern"
  type        = string
  default     = "*"
}

variable "include_stopped" {
  description = "Include stopped instances in query"
  type        = bool
  default     = true
}

# =============================================================================
# AMI Query
# =============================================================================

variable "ami_owners" {
  description = "AMI owners to search (amazon, self, or account ID)"
  type        = list(string)
  default     = ["amazon"]
}

variable "ami_name_filter" {
  description = "AMI name filter pattern"
  type        = string
  default     = "al2023-ami-*-x86_64"  # Amazon Linux 2023
}

# =============================================================================
# Instance Creation Variables (for future use - map-based for_each)
# =============================================================================

variable "instances" {
  description = "Map of EC2 instances to create"
  type = map(object({
    instance_type     = string
    subnet_id         = string
    is_public         = bool
    security_group_ids = list(string)
    key_name          = optional(string)
    user_data         = optional(string)
    root_volume_size  = optional(number, 20)
    tags              = optional(map(string), {})
  }))
  default = {}
}

variable "default_instance_type" {
  description = "Default EC2 instance type"
  type        = string
  default     = "t3.micro"
}