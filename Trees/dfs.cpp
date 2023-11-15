#include <iostream>
#include <bits/stdc++.h>

using namespace std;

struct Node{
        int data;
        Node* left;
        Node* right;

        Node (int val){
            data = val;
            left = NULL;
            right = NULL;
        }
    };

void dfs(struct Node* node){
	if (node == NULL)
		return;
    stack <struct Node*> s;
    s.push(node);
    while(s.size()>0){
        Node* current = s.top();
        cout<<current->data;
        s.pop();
        if(current->right!=NULL){
            s.push(current->right);
        }
        if(current->left!=NULL){
            s.push(current->left);
        }
    }
}

int main(){
    struct Node *root = new Node(1);
    root->left = new Node(2);
    root->right = new Node(3);
    root->left->left = new Node(4);
    root->left->right = new Node(6);
    root->right->right = new Node(8);
    

    /*
            1
           / \
          2   3
         / \   \
        4   6   8
    */
    
   dfs(root);

    return 0;
}
