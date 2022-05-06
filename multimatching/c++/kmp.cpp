#include <iostream>
using namespace std;

void checkPattern(string pattern, string text)
{
    // printf("pattern: %s\n", pattern.c_str());
    // printf("text: %s\n", text.c_str());

    size_t pattern_hash = hash<string>{}(pattern);
    // printf("pattern hash: %ld\n", pattern_hash);

    int pattern_size = pattern.size();
    int upper_index = text.size() - pattern_size + 1; //+ 1;

    // printf("upper index: %d\n", upper_index);

    for (int i = 0; i < upper_index; i++)
    {
        string sub_text = text.substr(i, pattern_size);
        // printf("sub text: %s\n", sub_text.c_str());
        size_t sub_text_hash = hash<string>{}(sub_text);
        // printf("text hash: %ld\n", sub_text_hash);
        if (sub_text_hash == pattern_hash)
        {
            printf("%d ", i);
        }
    }

    printf("\n");
}

void inputIteration()
{
    // printf("Hello World\n");
    int num_patterns;

    // std::getline(std::cin, num_patterns);

    scanf("%d", &num_patterns);

    //discard characters until newline is found
    cin.ignore(); 
    
    string patterns[num_patterns];

    // printf("num patterns: %d", num_patterns); 

    // scan patterns
    for (int i = 0; i < num_patterns; i++)
    {
        // printf("getting line"); 
        std::getline(std::cin, patterns[i]);
        // cin >> patterns[i];
    }

    // print patterns
    /* for (int i = 0; i < num_patterns; i++)
    {
        std::cout << patterns[i];
        printf("\n");
    } */

    string text;
    std::getline(std::cin, text);
    // cin >> text;

    for (int i = 0; i < num_patterns; i++)
    {
        checkPattern(patterns[i], text);
    }
}

int main()
{
    while (std::cin)
    {
        // printf("here"); 
        inputIteration(); 
    }
}