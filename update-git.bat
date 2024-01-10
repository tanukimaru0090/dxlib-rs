@REM cargo-git.bat
@REM cargo check and git command 
:git_err
	@if %errorlevel% equ 1 (
		echo git commandに失敗しました
		@del "./.git/index.lock" 
		echo index.lockを削除しました
		echo 再度git commandを試します。
		@call %git_command% 
	)
:git_err_eof 



:main

@setlocal
@set git_command=git add . ^& git commit -m Update^&git push

@REM *MainEntry*
	@REM cargo check 
	@cargo check

	@if %errorlevel% equ 0 (
	@call %git_command%
	@REM git コマンドが失敗したら
	@REM index.lockを削除してもう一度実行
		@if %errorlevel% equ 1 (
			@REM エラーメッセージ表示をし、git commandを再度行う
			@call :git_err
			@goto git_err_eof
		)
	)
@endlocal
@exit /b
:main_eof

@call main
goto main_eof
