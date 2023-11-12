#include <iostream>
#include <bits/stdc++.h>

using namespace std;

int fib(int num){
    int prev = 0;
    int prev2 = 1;
    for(int i=2; i<=num; i++){
        int curr = prev + prev2;
        prev = prev2;
        prev2 = curr;
    }
    return prev2;
}

int main(){
    int num;
    cin>>num;
    int ans = fib(num);
    cout<<"ANSWER IS: "<<ans<<"\n";
    return 0;
}
