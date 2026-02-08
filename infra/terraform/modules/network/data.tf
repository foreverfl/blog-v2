# =============================================================================
# VPCs
# =============================================================================

data "aws_vpcs" "all" {}

locals {
  # Assign vpc_id from variable or all VPCs
  target_vpc_ids = var.vpc_id != "" ? toset([var.vpc_id]) : toset(data.aws_vpcs.all.ids)
}

data "aws_vpc" "details" {
  for_each = local.target_vpc_ids
  id       = each.value
}

# =============================================================================
# Availability Zones
# =============================================================================

data "aws_availability_zones" "available" {
  state = "available"
}

# =============================================================================
# Subnets (per VPC)
# =============================================================================

data "aws_subnets" "by_vpc" {
  for_each = local.target_vpc_ids

  filter {
    name   = "vpc-id"
    values = [each.value]
  }
}

data "aws_subnets" "public_by_vpc" {
  for_each = local.target_vpc_ids

  filter {
    name   = "vpc-id"
    values = [each.value]
  }

  filter {
    name   = "map-public-ip-on-launch"
    values = ["true"]
  }
}

data "aws_subnets" "private_by_vpc" {
  for_each = local.target_vpc_ids

  filter {
    name   = "vpc-id"
    values = [each.value]
  }

  filter {
    name   = "map-public-ip-on-launch"
    values = ["false"]
  }
}

# Subnet details (combined subnets of all VPCs)
locals {
  all_subnet_ids = toset(flatten([
    for vpc_id in local.target_vpc_ids : data.aws_subnets.by_vpc[vpc_id].ids
  ]))
}

data "aws_subnet" "details" {
  for_each = local.all_subnet_ids
  id       = each.value
}

# Effective route table per subnet (explicit association or main route table)
data "aws_route_table" "by_subnet" {
  for_each  = local.all_subnet_ids
  subnet_id = each.value
}

# =============================================================================
# NAT Gateways (per VPC)
# =============================================================================

data "aws_nat_gateways" "by_vpc" {
  for_each = local.target_vpc_ids

  filter {
    name   = "vpc-id"
    values = [each.value]
  }

  filter {
    name   = "state"
    values = ["available", "pending"]
  }
}

# =============================================================================
# Route Tables (per VPC)
# =============================================================================

data "aws_route_tables" "by_vpc" {
  for_each = local.target_vpc_ids

  filter {
    name   = "vpc-id"
    values = [each.value]
  }
}

locals {
  all_route_table_ids = toset(flatten([
    for vpc_id in local.target_vpc_ids : data.aws_route_tables.by_vpc[vpc_id].ids
  ]))
}

data "aws_route_table" "details" {
  for_each       = local.all_route_table_ids
  route_table_id = each.value
}