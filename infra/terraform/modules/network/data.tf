# =============================================================================
# VPCs
# =============================================================================

data "aws_vpcs" "all" {}

locals {
  # vpc_id 지정 → 그것만 / 비어있으면 → 전부
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
# Subnets (VPC별)
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

# 서브넷 상세 (전체 VPC의 서브넷을 합쳐서 조회)
locals {
  all_subnet_ids = toset(flatten([
    for vpc_id in local.target_vpc_ids : data.aws_subnets.by_vpc[vpc_id].ids
  ]))
}

data "aws_subnet" "details" {
  for_each = local.all_subnet_ids
  id       = each.value
}

# =============================================================================
# NAT Gateways (VPC별)
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
# Route Tables (VPC별)
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