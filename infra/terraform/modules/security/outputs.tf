# =============================================================================
# Security Groups Summary
# =============================================================================

output "security_groups_summary" {
  description = "Security Groups summary"
  value = {
    count = length(data.aws_security_groups.all.ids)
    ids   = data.aws_security_groups.all.ids
  }
}

output "security_group_details" {
  description = "Detailed Security Group information"
  value = {
    for id, sg in data.aws_security_group.details : id => {
      id          = sg.id
      name        = sg.name
      description = sg.description
      vpc_id      = sg.vpc_id
      tags        = sg.tags
    }
  }
}

# =============================================================================
# 주요 Security Group IDs (발견된 경우)
# =============================================================================

output "web_security_group_ids" {
  description = "Web/HTTP related security group IDs"
  value       = data.aws_security_groups.web.ids
}

output "ssh_security_group_ids" {
  description = "SSH/Bastion related security group IDs"
  value       = data.aws_security_groups.ssh.ids
}

output "database_security_group_ids" {
  description = "Database related security group IDs"
  value       = data.aws_security_groups.database.ids
}

# =============================================================================
# IAM Information
# =============================================================================

output "ec2_iam_roles" {
  description = "IAM roles available for EC2"
  value = {
    names = data.aws_iam_roles.ec2_roles.names
    arns  = data.aws_iam_roles.ec2_roles.arns
  }
}

output "ssm_policy_arn" {
  description = "SSM Managed Instance Core policy ARN"
  value       = data.aws_iam_policy.ssm_managed_instance.arn
}

# =============================================================================
# Security Summary (한눈에 보기)
# =============================================================================

output "security_overview" {
  description = "Overall security configuration overview"
  value = {
    total_security_groups = length(data.aws_security_groups.all.ids)
    web_sgs_found         = length(data.aws_security_groups.web.ids)
    ssh_sgs_found         = length(data.aws_security_groups.ssh.ids)
    database_sgs_found    = length(data.aws_security_groups.database.ids)
    ec2_roles_found       = length(data.aws_iam_roles.ec2_roles.names)
  }
}