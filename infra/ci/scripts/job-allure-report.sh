#!/bin/bash


ALLURE_RESULTS='./backend/allure-results'
ALLURE_REPORT='./backend/allure-report'

aws s3 sync \
    "s3://${S3_BUCKET}/${CI_MERGE_REQUEST_SOURCE_BRANCH_NAME}/latest/history" \
    ${ALLURE_RESULTS}/history

BUILD_ORDER=$(aws s3 ls "s3://${S3_BUCKET}/${CI_MERGE_REQUEST_SOURCE_BRANCH_NAME}" | wc -l)

cat <<EOF > ${ALLURE_RESULTS}/environment.properties
branch=${CI_MERGE_REQUEST_SOURCE_BRANCH_NAME}
build_number=${CI_PIPELINE_ID}
EOF

cat <<EOF > ${ALLURE_RESULTS}/executor.json
{
  "name": "${CI_RUNNER_ID}",
  "type": "gitlab",
  "buildUrl": "${CI_PIPELINE_URL}",
  "buildOrder": ${BUILD_ORDER},
  "buildName": "${CI_COMMIT_TITLE}",
  "reportUrl": "https://${S3_BUCKET}/${CI_MERGE_REQUEST_SOURCE_BRANCH_NAME}/${CI_COMMIT_SHORT_SHA}"
}
EOF


ls ${ALLURE_RESULTS} -la

cat ${ALLURE_RESULTS}/executor.json

xmlstarlet ed -L -P \
    -i '/testsuites/testsuite[not(@timestamp)]' \
    -t attr -n timestamp \
    -v "$(xmlstarlet sel -t -v '/testsuites/@timestamp' ${ALLURE_RESULTS}/unit.xml)" \
    ${ALLURE_RESULTS}/unit.xml

xmlstarlet ed -L -P \
    -i '/testsuites/testsuite[not(@timestamp)]' \
    -t attr -n timestamp \
    -v "$(xmlstarlet sel -t -v '/testsuites/@timestamp' ${ALLURE_RESULTS}/integration.xml)" \
    ${ALLURE_RESULTS}/integration.xml

xmlstarlet ed -L -P \
    -i '/testsuites/testsuite[not(@timestamp)]' \
    -t attr -n timestamp \
    -v "$(xmlstarlet sel -t -v '/testsuites/@timestamp' ${ALLURE_RESULTS}/e2e.xml)" \
    ${ALLURE_RESULTS}/e2e.xml

allure --version
allure --verbose generate ${ALLURE_RESULTS} --output ${ALLURE_REPORT} --clean

aws s3 sync  ${ALLURE_REPORT} \
    "s3://${S3_BUCKET}/${CI_MERGE_REQUEST_SOURCE_BRANCH_NAME}/latest"

aws s3 sync  ${ALLURE_REPORT} \
    "s3://${S3_BUCKET}/${CI_MERGE_REQUEST_SOURCE_BRANCH_NAME}/${CI_COMMIT_SHORT_SHA}"

cat ${ALLURE_REPORT}/widgets/executors.json

echo "https://${S3_BUCKET}/${CI_MERGE_REQUEST_SOURCE_BRANCH_NAME}/latest"
