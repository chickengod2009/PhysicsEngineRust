#include "test.h"
#include <iostream>

extern "C"{
JNIEXPORT void JNICALL Java_App_hello
  (JNIEnv *, jobject){
    std::cout<<"k";
  }
}