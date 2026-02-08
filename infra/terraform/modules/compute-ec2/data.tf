# =============================================================================
# EC2 Instances Query
# =============================================================================

# If include_stopped is true, include both running and stopped instances.
# Otherwise, only include running instances.
locals {
  instance_states = var.include_stopped ? ["running", "stopped"] : ["running"]
}

# Fetch all EC2 instance IDs in the given VPC with the desired states.
data "aws_instances" "all" {
  filter {
    name   = "vpc-id"
    values = [var.vpc_id]
  }

  filter {
    name   = "instance-state-name"
    values = local.instance_states
  }
}

# Detailed information for each individual instance
data "aws_instance" "details" {
  for_each    = toset(data.aws_instances.all.ids)
  instance_id = each.value
}

# =============================================================================
# Public/Private Instances Classification
# =============================================================================

data "aws_instances" "public" {
  filter {
    name   = "vpc-id"
    values = [var.vpc_id]
  }

  filter {
    name   = "instance-state-name"
    values = local.instance_states
  }

  filter {
    name   = "network-interface.association.public-ip"
    values = ["*"]
  }
}

data "aws_instances" "private" {
  filter {
    name   = "vpc-id"
    values = [var.vpc_id]
  }

  filter {
    name   = "instance-state-name"
    values = local.instance_states
  }
}

# =============================================================================
# Latest AMI Query (Amazon Linux 2023)
# =============================================================================

data "aws_ami" "latest" {
  most_recent = true
  owners      = var.ami_owners

  filter {
    name   = "name"
    values = [var.ami_name_filter]
  }

  filter {
    name   = "virtualization-type"
    values = ["hvm"]
  }

  filter {
    name   = "root-device-type"
    values = ["ebs"]
  }

  filter {
    name   = "architecture"
    values = ["x86_64"]
  }
}

# Amazon Linux 2 (reference)
data "aws_ami" "amazon_linux_2" {
  most_recent = true
  owners      = ["amazon"]

  filter {
    name   = "name"
    values = ["amzn2-ami-hvm-*-x86_64-gp2"]
  }
}

# Ubuntu 22.04 LTS (reference)
data "aws_ami" "ubuntu" {
  most_recent = true
  owners      = ["099720109477"]  # Canonical

  filter {
    name   = "name"
    values = ["ubuntu/images/hvm-ssd/ubuntu-jammy-22.04-amd64-server-*"]
  }
}

# =============================================================================
# Note: Key Pairs
# =============================================================================
# aws_key_pairs data source doesn't exist.
# Use AWS CLI: aws ec2 describe-key-pairs
# =============================================================================