data "aws_caller_identity" "current" {}

resource "aws_iam_role" "terraform_apply_role" {
  name = "budget_buddy_terraform_deploy_role"
  assume_role_policy = jsonencode({
    "Version" : "2012-10-17",
    "Statement" : [
      {
        "Effect" : "Allow",
        "Principal" : {
          "Federated" : "arn:aws:iam::${data.aws_caller_identity.current.account_id}:oidc-provider/token.actions.githubusercontent.com"
        },
        "Action" : "sts:AssumeRoleWithWebIdentity",
        "Condition" : {
          "StringLike" : {
            "token.actions.githubusercontent.com:sub" : "repo:AmaanIbnNasar/budget_buddy:*"
          },
          "StringEquals" : {
            "token.actions.githubusercontent.com:aud" : "sts.amazonaws.com"
          }
        }
      }
    ]
    }
  )
}

resource "aws_iam_policy" "apply_policy" {
  name = "budget_buddy_terraform_apply_policy"
  policy = jsonencode({
    "Version" : "2012-10-17",
    "Statement" : [
      {
        "Effect" : "Allow",
        "Action" : [
          "s3:*",
          "lambda:*",
          "dynamodb:*",
          "iam:*"
        ],
        "Resource" : "*"
      }
    ]
    }
  )
}

resource "aws_iam_role_policy_attachment" "terraform_apply_policy_attachment" {
  policy_arn = aws_iam_policy.apply_policy.arn
  role       = aws_iam_role.terraform_apply_role.name
}

resource "aws_iam_policy" "lambda-service-policy" {
  name = "lambda-service-policy"
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Action = [
          "lambda:GetFunction",
          "lambda:GetLayerVersion",
          "lambda:CreateFunction",
          "lambda:UpdateFunctionCode",
          "lambda:UpdateFunctionConfiguration",
          "lambda:PublishVersion",
          "lambda:TagResource"
        ]
        Resource = [
          "arn:aws:lambda:eu-west-2:${data.aws_caller_identity.current.account_id}:function:budget_buddy",
        ]
      }
    ]
  })
}
resource "aws_iam_role_policy_attachment" "update_lambda_policy_attachment" {
  role       = aws_iam_role.terraform_apply_role.name
  policy_arn = aws_iam_policy.lambda-service-policy.arn
}

output "aws_access_key_id" {
  value = aws_iam_access_key.lambda-service-user.id
}

output "aws_secret_access_key" {
  value     = aws_iam_access_key.lambda-service-user.secret
  sensitive = true
}
