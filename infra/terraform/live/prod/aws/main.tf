# =============================================================================
# Prod AWS Infrastructure
# =============================================================================
# Assembles 4 modules for complete infrastructure query/management
# =============================================================================

# =============================================================================
# 1. Network Module
# =============================================================================
# VPC, Subnets, Internet Gateway, NAT Gateway, Route Tables

module "network" {
  source = "../../../modules/network"

  vpc_id     = var.vpc_id
  enable_nat = var.enable_nat
}

# =============================================================================
# 2. Security Module
# =============================================================================
# Security Groups, IAM Roles

module "security" {
  source = "../../../modules/security"

  vpc_id            = module.network.vpc_id
  allowed_ssh_cidrs = var.allowed_ssh_cidrs
}

# =============================================================================
# 3. Compute-EC2 Module
# =============================================================================
# EC2 Instances, AMIs, Key Pairs

module "compute" {
  source = "../../../modules/compute-ec2"

  vpc_id          = module.network.vpc_id
  include_stopped = var.include_stopped_instances
}

# =============================================================================
# 4. Observability Module
# =============================================================================
# CloudWatch Logs, Alarms, SNS, SSM

module "observability" {
  source = "../../../modules/observability"

  log_group_prefix = var.log_group_prefix
  ec2_instance_ids = module.compute.instances_summary.instance_ids
}

# =============================================================================
# Outputs - Network
# =============================================================================

output "network_vpc" {
  description = "VPC information"
  value       = module.network.vpc
}

output "network_subnets" {
  description = "Subnets summary"
  value       = module.network.subnets_summary
}

output "network_nat_status" {
  description = "NAT Gateway status"
  value = {
    enabled     = module.network.nat_enabled
    gateway_ids = module.network.nat_gateways.ids
  }
}

output "network_availability_zones" {
  description = "Available AZs"
  value       = module.network.availability_zones
}

# =============================================================================
# Outputs - Security
# =============================================================================

output "security_overview" {
  description = "Security configuration overview"
  value       = module.security.security_overview
}

output "security_groups" {
  description = "Security groups detail"
  value       = module.security.security_group_details
}

# =============================================================================
# Outputs - Compute
# =============================================================================

output "compute_overview" {
  description = "Compute status overview"
  value       = module.compute.compute_overview
}

output "compute_instances" {
  description = "EC2 instances detail"
  value       = module.compute.instance_details
}

output "compute_public_instances" {
  description = "Public EC2 instances"
  value       = module.compute.public_instances
}

output "compute_private_instances" {
  description = "Private EC2 instances"
  value       = module.compute.private_instances
}

output "compute_available_amis" {
  description = "Available AMI options"
  value       = module.compute.available_amis
}

# =============================================================================
# Outputs - Observability
# =============================================================================

output "observability_overview" {
  description = "Observability status overview"
  value       = module.observability.observability_overview
}

output "observability_log_groups" {
  description = "CloudWatch log groups"
  value       = module.observability.log_groups
}

# =============================================================================
# Quick Status (at a glance)
# =============================================================================

output "infrastructure_status" {
  description = "Quick infrastructure status"
  value = {
    # Network
    vpc_id          = module.network.vpc_id
    public_subnets  = length(module.network.public_subnet_ids)
    private_subnets = length(module.network.private_subnet_ids)
    nat_enabled     = module.network.nat_enabled

    # Security
    security_groups = module.security.security_overview.total_security_groups

    # Compute
    total_instances = module.compute.compute_overview.total_instances
    running_public  = module.compute.compute_overview.running_public
    running_private = module.compute.compute_overview.running_private

    # Observability
    log_groups = module.observability.observability_overview.log_groups_count
    region     = module.observability.observability_overview.region
  }
}