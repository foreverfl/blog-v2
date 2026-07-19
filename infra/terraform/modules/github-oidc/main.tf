# =============================================================================
# GitHub Actions OIDC Federation
# =============================================================================
# Lets GitHub Actions assume a read-only role without long-lived access keys.
# =============================================================================

resource "aws_iam_openid_connect_provider" "github" {
  url            = "https://token.actions.githubusercontent.com"
  client_id_list = ["sts.amazonaws.com"]
  # AWS validates GitHub's cert against trusted root CAs; thumbprint is a
  # required legacy field, not the actual trust anchor.
  thumbprint_list = ["6938fd4d98bab03faadb97b34396831e3780aea1"]
}

data "aws_iam_policy_document" "assume_from_github" {
  statement {
    effect  = "Allow"
    actions = ["sts:AssumeRoleWithWebIdentity"]

    principals {
      type        = "Federated"
      identifiers = [aws_iam_openid_connect_provider.github.arn]
    }

    condition {
      test     = "StringEquals"
      variable = "token.actions.githubusercontent.com:aud"
      values   = ["sts.amazonaws.com"]
    }

    condition {
      test     = "StringLike"
      variable = "token.actions.githubusercontent.com:sub"
      # Scheduled workflows always run on the default branch, so main-only
      # is enough — no wildcard access for PR branches or forks.
      values = [for repo in var.github_repos : "repo:${repo}:ref:refs/heads/main"]
    }
  }
}

resource "aws_iam_role" "github_actions_readonly" {
  name               = var.role_name
  assume_role_policy = data.aws_iam_policy_document.assume_from_github.json
}

data "aws_iam_policy_document" "readonly" {
  statement {
    effect = "Allow"
    actions = [
      "ce:GetCostAndUsage",
      "ce:GetAnomalies",
      "ec2:DescribeInstances",
    ]
    # Cost Explorer and EC2 Describe do not support resource-level scoping
    resources = ["*"]
  }
}

resource "aws_iam_role_policy" "readonly" {
  name   = "${var.role_name}-readonly"
  role   = aws_iam_role.github_actions_readonly.id
  policy = data.aws_iam_policy_document.readonly.json
}
