#include <iostream>
#include <bits/stdc++.h>

using namespace std;

int fib(int n, vector<int>& v){
    if(n<=1){
        return n;
    }
    else if(v[n]!=-1){
        return v[n];
    }
    else{
        return v[n] = fib(n-1, v) + fib(n-2, v);
    }
}

int main(){
    int num;
    cin>>num;
    vector <int> v (num+1,-1);
    int ans = fib(num, v);
    cout<<"ANSWER IS: "<<ans<<"\n";
    return 0;
}
