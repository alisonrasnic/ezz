format PE console 4.0
entry start

include 'win32a.inc'
	start:
		invoke GetStdHandle,STD_OUTPUT_HANDLE\ninvoke WriteConsole,eax,hello,14,NULL,NULL\n\t\t\tinvoke ExitProcess,0"