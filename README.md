# heart beat

#### 사용예시 1

30초 주기로 CURL 명령어 실행하기 (SIGALARM)

```bash
$ heat -i 30 curl -sf localhost/health_check
```

(curl 명령어에서 -f 옵션은 Response Status Code 가 200이 아니면, 0이 아닌 exit code를 반환합니다.)

여기서 "-i 30"은 옵션, "-sf localhost/health_check"은 검사 명령입니다.

**예시 결과**는 이렇습니다

```shell
1669085100: OK
1669085130: OK
1669085160: Failed: Exit Code 22, details in heading
1669085190: OK
```

앞의 1669085100은 시간(초)이고 : 뒤 부분은 각회차의 실행, 실패할 경우의 Exit code와 로그파일의 위치를 안내합니다.

#### 사용예시 2

shell script를 지정하여 사용하기

```bash
$ heat -i 30 -s ./check
```

전의 검사부분을 스크립트로 작성하면 위와 같이 사용할 수 있습니다.

##### shell script 내용 확인

```bash
$ cat ./check
```

```shell
#!/bin/bash
curl -sf localhost/health_check
```

이때, 주의할 것은 shell script는 executable해야 해요. 그렇지 않으면 실패처리를 하구요.
파일이 executable한 지 검사하는 방법은 다른 문서에 추가 작성하겠습니다!!
또, 검사 스크립트와 검사 명령어가 모두 지정되거나 그렇지 않은 경우 에러처리 또한 필요합니다.
