#include <iostream>
#include <bits/stdc++.h>

using namespace std;

int uniquePaths(int r, int c, vector <vector <int>> dp){
    if(r == 0 && c == 0){
        return 1;
    }
    if(r<0 || c<0){
        return 0;
    }
    if(dp[r][c]!=-1){
        return dp[r][c];
    }
    int up = uniquePaths(r,c-1, dp);
    int left = uniquePaths(r-1,c, dp);
    return dp[r][c] = up+left;
}

int main(){
    int r = 6, c = 6;
    vector <vector <int>> dp(r, vector <int> (c,-1));
    cout<<uniquePaths(r-1,c-1,dp);
    return 0;
}
