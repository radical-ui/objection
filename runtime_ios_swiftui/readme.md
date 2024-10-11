```shell
export APP_ID="MyApp"
export APP_NAME="MyName"
export BUNDLE_IDENTIFIER="com.example.my-app"

sd --fixed-strings -f c com.example.objection-app $BUNDLE_IDENTIFIER ObjectionApp.xcodeproj/project.pbxproj
sd --fixed-strings -f c ObjectionApp $APP_ID ObjectionApp.xcodeproj/project.pbxproj
sd --fixed-strings -f c ObjectionName $APP_NAME ObjectionApp.xcodeproj/project.pbxproj

mv ObjectionApp/ObjectionApp.entitlements ObjectionApp/$APP_ID.entitlements
mv ObjectionApp.xcodeproj $APP_ID.xcodeproj
mv ObjectionApp $APP_ID
```
