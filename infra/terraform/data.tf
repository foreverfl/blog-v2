# =============================================================================
# Account & Region Information
# =============================================================================

data "aws_caller_identity" "current" {}

data "aws_region" "current" {}

data "aws_availability_zones" "available" {
  state = "available"
}

# =============================================================================
# VPC Information
# =============================================================================

# Get all VPCs
data "aws_vpcs" "all" {}

# Get VPC details (using first VPC or filtered by name)
data "aws_vpc" "main" {
  count = length(data.aws_vpcs.all.ids) > 0 ? 1 : 0

  filter {
    name   = "vpc-id"
    values = [data.aws_vpcs.all.ids[0]]
  }
}

# Get all subnets in the VPC
data "aws_subnets" "all" {
  count = length(data.aws_vpcs.all.ids) > 0 ? 1 : 0

  filter {
    name   = "vpc-id"
    values = [data.aws_vpcs.all.ids[0]]
  }
}

# Get public subnets (subnets with map_public_ip_on_launch = true)
data "aws_subnets" "public" {
  count = length(data.aws_vpcs.all.ids) > 0 ? 1 : 0

  filter {
    name   = "vpc-id"
    values = [data.aws_vpcs.all.ids[0]]
  }

  filter {
    name   = "map-public-ip-on-launch"
    values = ["true"]
  }
}

# Get private subnets
data "aws_subnets" "private" {
  count = length(data.aws_vpcs.all.ids) > 0 ? 1 : 0

  filter {
    name   = "vpc-id"
    values = [data.aws_vpcs.all.ids[0]]
  }

  filter {
    name   = "map-public-ip-on-launch"
    values = ["false"]
  }
}

# Get subnet details
data "aws_subnet" "details" {
  for_each = length(data.aws_subnets.all) > 0 ? toset(data.aws_subnets.all[0].ids) : toset([])
  id       = each.value
}

# Internet Gateway
data "aws_internet_gateway" "main" {
  count = length(data.aws_vpcs.all.ids) > 0 ? 1 : 0

  filter {
    name   = "attachment.vpc-id"
    values = [data.aws_vpcs.all.ids[0]]
  }
}

# NAT Gateways
data "aws_nat_gateways" "all" {
  count = length(data.aws_vpcs.all.ids) > 0 ? 1 : 0

  filter {
    name   = "vpc-id"
    values = [data.aws_vpcs.all.ids[0]]
  }
}

# Route Tables
data "aws_route_tables" "all" {
  count = length(data.aws_vpcs.all.ids) > 0 ? 1 : 0

  filter {
    name   = "vpc-id"
    values = [data.aws_vpcs.all.ids[0]]
  }
}

# =============================================================================
# EC2 Instances
# =============================================================================

# Get all EC2 instances
data "aws_instances" "all" {
  instance_state_names = ["running", "stopped", "pending"]
}

# Get detailed info for each instance
data "aws_instance" "details" {
  for_each    = toset(data.aws_instances.all.ids)
  instance_id = each.value
}

# =============================================================================
# Security Groups
# =============================================================================

data "aws_security_groups" "all" {
  count = length(data.aws_vpcs.all.ids) > 0 ? 1 : 0

  filter {
    name   = "vpc-id"
    values = [data.aws_vpcs.all.ids[0]]
  }
}

data "aws_security_group" "details" {
  for_each = length(data.aws_security_groups.all) > 0 ? toset(data.aws_security_groups.all[0].ids) : toset([])
  id       = each.value
}

# =============================================================================
# EBS Volumes
# =============================================================================

data "aws_ebs_volumes" "all" {}

data "aws_ebs_volume" "details" {
  for_each = toset(data.aws_ebs_volumes.all.ids)

  filter {
    name   = "volume-id"
    values = [each.value]
  }
}

# =============================================================================
# Elastic IPs
# =============================================================================

data "aws_eips" "all" {}

