terraform {
  backend "s3" {
    bucket         = "budget-buddy-terraform-state"
    key            = "state.tf"
    region         = "eu-west-2"
    dynamodb_table = "budget-buddy-terraform-state-lock"
    encrypt        = true
  }
}
