#include <iostream>
#include <string>
#include <unordered_map>

bool CanSum(std::vector<int> arreglo, int objetivo){
    sort(arreglo.begin(), arreglo.end());
    bool status;
    int suma = 0;
    if(objetivo == 0){
        return true;
    }

    for(int i = 0; i < arreglo.size(); i++){
        if(arreglo[i] == objetivo){
            status = true;
        }
        int residuo = objetivo - suma;
        if(residuo != 0){
            status =false;
            suma = arreglo[i] + arreglo[i+1];
            residuo = objetivo - suma;
            

        }else{
        status = true;
        }
        
    }
    return status;


}

int main(){
    std::vector<int> arreglo = {2,3,3,6};
    int objetivo = 7;

    std::cout<<"Resultado: "<<CanSum(arreglo,objetivo)<<std::endl;

    return 0;
}

