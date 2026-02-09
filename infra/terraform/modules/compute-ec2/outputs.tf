# =============================================================================
# EC2 Instances Summary
# =============================================================================

output "instances_summary" {
  description = "EC2 instances summary"
  value = {
    total_count   = length(data.aws_instances.all.ids)
    public_count  = length(data.aws_instances.public.ids)
    private_count = length(data.aws_instances.all.ids) - length(data.aws_instances.public.ids)
    instance_ids  = data.aws_instances.all.ids
    public_ids    = data.aws_instances.public.ids
  }
}

output "instance_details" {
  description = "Detailed EC2 instance information"
  value = {
    for id, instance in data.aws_instance.details : id => {
      id                = instance.id
      name              = lookup(instance.tags, "Name", "unnamed")
      instance_type     = instance.instance_type
      state             = instance.instance_state
      availability_zone = instance.availability_zone
      subnet_id         = instance.subnet_id
      private_ip        = instance.private_ip
      public_ip         = instance.public_ip
      ami_id            = instance.ami
      key_name          = instance.key_name
      security_groups   = instance.vpc_security_group_ids
      iam_role          = instance.iam_instance_profile
      tags              = instance.tags
    }
  }
}

# =============================================================================
# Public/Private EC2 Instances
# =============================================================================

output "public_instances" {
  description = "Public EC2 instances (with public IP)"
  value = {
    for id, instance in data.aws_instance.details : id => {
      id         = instance.id
      name       = lookup(instance.tags, "Name", "unnamed")
      public_ip  = instance.public_ip
      private_ip = instance.private_ip
      state      = instance.instance_state
    } if instance.public_ip != ""
  }
}

output "private_instances" {
  description = "Private EC2 instances (no public IP)"
  value = {
    for id, instance in data.aws_instance.details : id => {
      id         = instance.id
      name       = lookup(instance.tags, "Name", "unnamed")
      private_ip = instance.private_ip
      state      = instance.instance_state
      subnet_id  = instance.subnet_id
    } if instance.public_ip == "" || instance.public_ip == null
  }
}

# =============================================================================
# AMI Information
# =============================================================================

output "latest_ami" {
  description = "Latest AMI matching the filter"
  value = {
    id            = data.aws_ami.latest.id
    name          = data.aws_ami.latest.name
    creation_date = data.aws_ami.latest.creation_date
    owner         = data.aws_ami.latest.owner_id
  }
}

output "available_amis" {
  description = "Available AMI options"
  value = {
    amazon_linux_2023 = {
      id   = data.aws_ami.latest.id
      name = data.aws_ami.latest.name
    }
    amazon_linux_2 = {
      id   = data.aws_ami.amazon_linux_2.id
      name = data.aws_ami.amazon_linux_2.name
    }
    ubuntu_22_04 = {
      id   = data.aws_ami.ubuntu.id
      name = data.aws_ami.ubuntu.name
    }
  }
}

# =============================================================================
# Note: Key Pairs
# =============================================================================
# aws_key_pairs data source doesn't exist.
# Use AWS CLI: aws ec2 describe-key-pairs
# =============================================================================

# =============================================================================
# Compute Overview (At a Glance)
# =============================================================================

output "compute_overview" {
  description = "Overall compute status"
  value = {
    total_instances   = length(data.aws_instances.all.ids)
    running_public    = length([for id, i in data.aws_instance.details : id if i.instance_state == "running" && i.public_ip != ""])
    running_private   = length([for id, i in data.aws_instance.details : id if i.instance_state == "running" && (i.public_ip == "" || i.public_ip == null)])
    stopped_instances = length([for id, i in data.aws_instance.details : id if i.instance_state == "stopped"])
  }
}