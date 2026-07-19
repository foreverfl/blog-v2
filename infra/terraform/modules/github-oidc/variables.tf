# =============================================================================
# GitHub OIDC Module Variables
# =============================================================================

variable "github_repos" {
  description = "GitHub repos (owner/name) allowed to assume the role"
  type        = list(string)
}

variable "role_name" {
  description = "Name of the IAM role assumed by GitHub Actions"
  type        = string
  default     = "github-actions-readonly"
}
