/*----------------------------------------------------------------------------------------------
This code can perform matrix multipication

Note: I did a bit of a silly thing while writing this code. Most of us define matrix by number
of rows and number of columns. But I defined this entire code based on Row Size and Column Size
it means, 

Row Size = Number of cols &
Col Size = Number of rows

I don't know what the freak was going through my head for the entire time of 22:00 to 06:00
But i'm happy this code works
----------------------------------------------------------------------------------------------*/

#include <stdio.h>

//Mat Sizes Input Function
void matSizeInput(int* row, int* col){
    printf("Enter the row size and col size of the matrix in following format rr.. cc..: ");
    scanf("%d %d", row, col);
    return;
}

//Matrix Print Function
void printMat(int r, int c, int mat[c][r]){
    for(int i = 0; i < c; ++i){
        for(int j = 0; j < r; ++j)
            printf("%d  ", mat[i][j]);
    printf("\n");
    }
    return;
}

//Matrix Values Input Function
void enterMat(
    int rowSize, 
    int colSize, 
    int mat[colSize][rowSize]
){

    for(int c = 0; c < colSize; c++){
        for(int r = 0; r < rowSize; r++){
            int val;
            printf("row: %d, col: %d = ", r+1, c+1);
            scanf("%d", &val); printf("\n");
            mat[c][r] = val;
        }
    }
    return;
}

//The Most time consumming pice of s function to make (it took me 3hrs to build)
//The Matrix Multiply Function
int matrixMultiply(
    int mat1R, 
    int mat1C, 
    int mat2R, 
    int mat2C, 
    int mat1[mat1C][mat1R], 
    int mat2[mat2C][mat2R],
    int result[mat1C][mat2R]
){

    if(mat1R != mat2C){       
        printf("These matrices cannot be multipied");
        return 1;
    }

    for(int k = 0; k < mat1C; ++k){                    //If a code works don't touch it
        int j = 0;                                     //                              -Devs
        for(int l = 0; l < mat2R; l++){
            int sum = 0;
            for(int m = 0; m < mat1R; m++)
            sum += mat1[k][m]*mat2[m][l];
            result[k][j++] = sum;
        }
        
    }


    return 0;
}

int main()
{
    int mat1RowSize, mat1ColSize;   //Mat 1 Sizes
    int mat2RowSize, mat2ColSize;   //Mat 2 Sizes

    printf("Enter Size of Matrix 1\n");
    matSizeInput(&mat1RowSize, &mat1ColSize); //passing the address of sizes

    int mat1[mat1ColSize][mat1RowSize];       //Defining the matrix 1

    enterMat(mat1RowSize, mat1ColSize, mat1); //Taking Input for the values of the matrix 1 from the user

    printf("Enter Size of Matrix 2\n");
    matSizeInput(&mat2RowSize, &mat2ColSize); //passing the address of sizes

    int mat2[mat2ColSize][mat2RowSize];       //Defining the matrix 2

    enterMat(mat2RowSize, mat2ColSize, mat2); //Taking Input for the values of the matrix 2 from the user

    
    int result[mat1ColSize][mat2RowSize];     //Defining the result matrix beacuse arrays can't be returned by functions 

    matrixMultiply(                           //Matrix multiply function can take these arguments and return the result 
        mat1RowSize,                          //to the result matrix written in the last
        mat1ColSize,                          //this function return 0 on successfull operation
        mat2RowSize,                          //if the matrix is not multipliable then it will return 1 and a error message
        mat2ColSize,
        mat1,
        mat2,
        result
    );

    printMat(                                 //Print Matrix Function
        mat2RowSize,                          //I know its a bit ugly for now for uneven values length
        mat1ColSize,                          //its the most basic matrix print function. just spits out the values one by one
        result
    );
    return 0;
}
