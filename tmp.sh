curl --user eef75cd772bb5672467f9fafc440437e165257f4: \
  --request POST \
  --form config=@.circleci/config.yml \
  --form notify=true \
    https://circleci.com/api/v1.1/project/github/BlueHotDog/buff/tree/master
