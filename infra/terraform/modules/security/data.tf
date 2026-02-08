# =============================================================================
# Security Groups
# =============================================================================

data "aws_security_groups" "all" {
  filter {
    name   = "vpc-id"
    values = [var.vpc_id]
  }
}

data "aws_security_group" "details" {
  for_each = toset(data.aws_security_groups.all.ids)
  id       = each.value
}

# =============================================================================
# Security Groups by Name Pattern
# =============================================================================

# Web/HTTP Security Group (if exists)
data "aws_security_groups" "web" {
  filter {
    name   = "vpc-id"
    values = [var.vpc_id]
  }

  filter {
    name   = "group-name"
    values = ["*web*", "*http*", "*Web*", "*HTTP*"]
  }
}

# SSH Security Group (if exists)
data "aws_security_groups" "ssh" {
  filter {
    name   = "vpc-id"
    values = [var.vpc_id]
  }

  filter {
    name   = "group-name"
    values = ["*ssh*", "*SSH*", "*bastion*", "*Bastion*"]
  }
}

# Database Security Group (if exists)
data "aws_security_groups" "database" {
  filter {
    name   = "vpc-id"
    values = [var.vpc_id]
  }

  filter {
    name   = "group-name"
    values = ["*db*", "*database*", "*rds*", "*DB*", "*Database*", "*RDS*"]
  }
}

# =============================================================================
# IAM Roles (EC2 related)
# =============================================================================

data "aws_iam_roles" "ec2_roles" {
  name_regex  = var.iam_role_name_prefix != "" ? "${var.iam_role_name_prefix}.*" : ".*"
  path_prefix = "/"
}

# =============================================================================
# Note: IAM Instance Profiles
# =============================================================================
# aws_iam_instance_profiles requires a specific role_name.
# Use AWS CLI: aws iam list-instance-profiles
# =============================================================================

# SSM Managed Policy
data "aws_iam_policy" "ssm_managed_instance" {
  name = "AmazonSSMManagedInstanceCore"
}