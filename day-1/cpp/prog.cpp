#include <iostream>
#include <fstream>
#include <vector>
#include <string>

using namespace std;

int SUM_TARGET = 2020;

vector<int> get_input() {
    vector<int> values;

    ifstream input;
    string linebuf;

    input.open("../input.txt");

    if (input.is_open()) {
        while (getline(input, linebuf)) {
            values.push_back(stoi(linebuf));
        }
        input.close();
    }

    return values;
}

int main() {
    vector<int> values = get_input();

    // Part 1

    int valI;
    int valJ;

    for (int i = 0; i < values.size(); i++) {
        for (int j = 0; j < values.size(); j++) {
            valI = values.at(i);
            valJ = values.at(j);

            if (valI + valJ == SUM_TARGET) {
                goto found_part_1;
            } 
        }
    }

found_part_1:
    cout << "Part 1: " << valI * valJ << '\n';

    int valP;
    int valQ;
    int valR;

    for (int p = 0; p < values.size(); p++) {
        for (int q = 0; q < values.size(); q++) {
            for (int r = 0; r < values.size(); r++) {
                valP = values.at(p);
                valQ = values.at(q);
                valR = values.at(r);

                if (valP + valQ + valR == SUM_TARGET) {
                    goto found_part_2;
                } 
            }
        }
    }

found_part_2:

    cout << "Part 2: " << valP * valQ * valR << '\n';

    return 0;
}
