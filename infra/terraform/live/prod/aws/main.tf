# =============================================================================
# Prod AWS Infrastructure
# =============================================================================

# =============================================================================
# 1. Network Module
# =============================================================================

module "network" {
  source = "../../../modules/network"

  vpc_id                  = var.vpc_id
  nat_enabled             = var.nat_enabled
  nat_subnet_id           = var.nat_subnet_id
  nat_private_route_table_ids = var.nat_private_route_table_ids
}

# =============================================================================
# 2. Security Module
# =============================================================================
module "security" {
  source = "../../../modules/security"

  vpc_id            = var.vpc_id
  allowed_ssh_cidrs = var.allowed_ssh_cidrs
}

# =============================================================================
# 3. Compute-EC2 Module
# =============================================================================
module "compute" {
  source = "../../../modules/compute-ec2"

  vpc_id          = var.vpc_id
  include_stopped = var.include_stopped_instances
}

# =============================================================================
# 4. Observability Module
# =============================================================================
module "observability" {
  source = "../../../modules/observability"

  log_group_prefix = var.log_group_prefix
  ec2_instance_ids = module.compute.instances_summary.instance_ids
}

# =============================================================================
# 1. Outputs - Network
# =============================================================================

output "network_vpcs" {
  description = "VPC details"
  value       = module.network.vpcs
}

output "network_subnet_details" {
  description = "Subnet details"
  value       = module.network.subnet_details
}

output "network_route_tables" {
  description = "Route table details"
  value       = module.network.route_table_details
}

output "network_availability_zones" {
  description = "Available AZs"
  value       = module.network.availability_zones
}

output "network_summary" {
  description = "Network summary"
  value       = module.network.summary
}

output "network_nat_gateway" {
  description = "NAT Gateway details"
  value       = module.network.nat_gateway
}

# =============================================================================
# 2. Outputs - Security
# =============================================================================

# output "security_groups_summary" {
#   description = "Security Groups summary"
#   value       = module.security.security_groups_summary
# }

# output "security_group_details" {
#   description = "Detailed Security Group information"
#   value       = module.security.security_group_details
# }

# output "security_web_sg_ids" {
#   description = "Web/HTTP related security group IDs"
#   value       = module.security.web_security_group_ids
# }

# output "security_ssh_sg_ids" {
#   description = "SSH/Bastion related security group IDs"
#   value       = module.security.ssh_security_group_ids
# }

# output "security_database_sg_ids" {
#   description = "Database related security group IDs"
#   value       = module.security.database_security_group_ids
# }

# output "security_ec2_iam_roles" {
#   description = "IAM roles available for EC2"
#   value       = module.security.ec2_iam_roles
# }

# output "security_ssm_policy_arn" {
#   description = "SSM Managed Instance Core policy ARN"
#   value       = module.security.ssm_policy_arn
# }

# output "security_overview" {
#   description = "Overall security configuration overview"
#   value       = module.security.security_overview
# }

# =============================================================================
# 3. Outputs - Compute
# =============================================================================

# output "compute_instances_summary" {
#   description = "EC2 instances summary"
#   value       = module.compute.instances_summary
# }

# output "compute_instance_details" {
#   description = "Detailed EC2 instance information"
#   value       = module.compute.instance_details
# }

# output "compute_public_instances" {
#   description = "Public EC2 instances"
#   value       = module.compute.public_instances
# }

# output "compute_private_instances" {
#   description = "Private EC2 instances"
#   value       = module.compute.private_instances
# }

# output "compute_latest_ami" {
#   description = "Latest AMI matching the filter"
#   value       = module.compute.latest_ami
# }

# output "compute_available_amis" {
#   description = "Available AMI options"
#   value       = module.compute.available_amis
# }

# output "compute_overview" {
#   description = "Overall compute status"
#   value       = module.compute.compute_overview
# }

# =============================================================================
# 4. Outputs - Observability
# =============================================================================

# output "observability_log_groups" {
#   description = "CloudWatch Log Groups"
#   value       = module.observability.log_groups
# }

# output "observability_ssm_parameters" {
#   description = "SSM Parameter Store entries"
#   value       = module.observability.ssm_parameters
# }

# output "observability_account_info" {
#   description = "Current AWS account and region info"
#   value       = module.observability.account_info
# }

# output "observability_overview" {
#   description = "Overall observability status"
#   value       = module.observability.observability_overview
# }

# output "observability_log_insights_queries" {
#   description = "Useful CloudWatch Logs Insights query examples"
#   value       = module.observability.useful_log_insights_queries
# }