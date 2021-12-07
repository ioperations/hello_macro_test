#!/usr/bin/env make
# $(error ${ccred} error message ${ccwhite})
# $(info ${ccyellow} info message ${ccwhite})


#check if file exists
#ifneq ("$(wildcard  FILENAME)","")
##file exists; use space insted of tab here
#endif 

#use ${LINENO} to locate the error message location

default:
	cd hello_macro_test && cargo build
