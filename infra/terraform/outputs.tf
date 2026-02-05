# =============================================================================
# Account & Region Outputs
# =============================================================================

output "account_info" {
  description = "AWS Account Information"
  value = {
    account_id = data.aws_caller_identity.current.account_id
    caller_arn = data.aws_caller_identity.current.arn
    user_id    = data.aws_caller_identity.current.user_id
    region     = data.aws_region.current.name
  }
}

output "availability_zones" {
  description = "Available AZs in the region"
  value       = data.aws_availability_zones.available.names
}

# =============================================================================
# VPC Outputs
# =============================================================================

output "vpc_info" {
  description = "VPC Information"
  value = length(data.aws_vpc.main) > 0 ? {
    vpc_id               = data.aws_vpc.main[0].id
    cidr_block           = data.aws_vpc.main[0].cidr_block
    enable_dns_hostnames = data.aws_vpc.main[0].enable_dns_hostnames
    enable_dns_support   = data.aws_vpc.main[0].enable_dns_support
    instance_tenancy     = data.aws_vpc.main[0].instance_tenancy
    main_route_table_id  = data.aws_vpc.main[0].main_route_table_id
    owner_id             = data.aws_vpc.main[0].owner_id
    tags                 = data.aws_vpc.main[0].tags
  } : null
}

output "subnets_summary" {
  description = "Subnets Summary"
  value = {
    total_count   = length(data.aws_subnets.all) > 0 ? length(data.aws_subnets.all[0].ids) : 0
    public_count  = length(data.aws_subnets.public) > 0 ? length(data.aws_subnets.public[0].ids) : 0
    private_count = length(data.aws_subnets.private) > 0 ? length(data.aws_subnets.private[0].ids) : 0
    public_ids    = length(data.aws_subnets.public) > 0 ? data.aws_subnets.public[0].ids : []
    private_ids   = length(data.aws_subnets.private) > 0 ? data.aws_subnets.private[0].ids : []
  }
}

output "subnet_details" {
  description = "Detailed subnet information"
  value = {
    for id, subnet in data.aws_subnet.details : id => {
      subnet_id                = subnet.id
      availability_zone        = subnet.availability_zone
      cidr_block               = subnet.cidr_block
      available_ip_count       = subnet.available_ip_address_count
      map_public_ip_on_launch  = subnet.map_public_ip_on_launch
      is_public                = subnet.map_public_ip_on_launch
      tags                     = subnet.tags
    }
  }
}

output "internet_gateway" {
  description = "Internet Gateway Information"
  value = length(data.aws_internet_gateway.main) > 0 ? {
    igw_id = data.aws_internet_gateway.main[0].id
    vpc_id = data.aws_internet_gateway.main[0].attachments[0].vpc_id
    state  = data.aws_internet_gateway.main[0].attachments[0].state
    tags   = data.aws_internet_gateway.main[0].tags
  } : null
}

output "nat_gateways" {
  description = "NAT Gateways"
  value = length(data.aws_nat_gateways.all) > 0 ? {
    count = length(data.aws_nat_gateways.all[0].ids)
    ids   = data.aws_nat_gateways.all[0].ids
  } : null
}

output "route_tables" {
  description = "Route Tables"
  value = length(data.aws_route_tables.all) > 0 ? {
    count = length(data.aws_route_tables.all[0].ids)
    ids   = data.aws_route_tables.all[0].ids
  } : null
}

# =============================================================================
# EC2 Outputs
# =============================================================================

output "ec2_instances_summary" {
  description = "EC2 Instances Summary"
  value = {
    total_count = length(data.aws_instances.all.ids)
    instance_ids = data.aws_instances.all.ids
  }
}

output "ec2_instance_details" {
  description = "Detailed EC2 instance information"
  value = {
    for id, instance in data.aws_instance.details : id => {
      instance_id           = instance.id
      instance_type         = instance.instance_type
      instance_state        = instance.instance_state
      ami_id                = instance.ami
      availability_zone     = instance.availability_zone

      # Network
      public_ip             = instance.public_ip
      private_ip            = instance.private_ip
      public_dns            = instance.public_dns
      private_dns           = instance.private_dns
      vpc_id                = instance.vpc_security_group_ids
      subnet_id             = instance.subnet_id
      security_groups       = instance.vpc_security_group_ids

      # Storage
      root_block_device     = instance.root_block_device
      ebs_block_devices     = instance.ebs_block_device

      # Metadata
      key_name              = instance.key_name
      iam_instance_profile  = instance.iam_instance_profile
      monitoring            = instance.monitoring
      tags                  = instance.tags

      # Lifecycle
      launch_time           = instance.launch_time
      disable_api_termination = instance.disable_api_termination
    }
  }
}

# =============================================================================
# Security Group Outputs
# =============================================================================

output "security_groups_summary" {
  description = "Security Groups Summary"
  value = length(data.aws_security_groups.all) > 0 ? {
    total_count = length(data.aws_security_groups.all[0].ids)
    ids         = data.aws_security_groups.all[0].ids
  } : null
}

output "security_group_details" {
  description = "Detailed Security Group information"
  value = {
    for id, sg in data.aws_security_group.details : id => {
      id          = sg.id
      name        = sg.name
      description = sg.description
      vpc_id      = sg.vpc_id
      arn         = sg.arn
      tags        = sg.tags
    }
  }
}

# =============================================================================
# EBS Volume Outputs
# =============================================================================

output "ebs_volumes_summary" {
  description = "EBS Volumes Summary"
  value = {
    total_count = length(data.aws_ebs_volumes.all.ids)
    volume_ids  = data.aws_ebs_volumes.all.ids
  }
}

output "ebs_volume_details" {
  description = "Detailed EBS Volume information"
  value = {
    for id, vol in data.aws_ebs_volume.details : id => {
      volume_id           = vol.id
      availability_zone   = vol.availability_zone
      size                = vol.size
      volume_type         = vol.volume_type
      iops                = vol.iops
      throughput          = vol.throughput
      encrypted           = vol.encrypted
      kms_key_id          = vol.kms_key_id
      snapshot_id         = vol.snapshot_id
      tags                = vol.tags
    }
  }
}

# =============================================================================
# Elastic IP Outputs
# =============================================================================

output "elastic_ips" {
  description = "Elastic IPs"
  value = {
    count = length(data.aws_eips.all.allocation_ids)
    allocation_ids = data.aws_eips.all.allocation_ids
    public_ips     = data.aws_eips.all.public_ips
  }
}

