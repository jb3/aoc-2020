from dataclasses import dataclass


@dataclass
class PasswordPolicy:
    leftmost: int
    rightmost: int
    character: str

    def meets_repetition_policy(self, password: str):
        appearances = password.count(self.character)

        return self.leftmost <= appearances <= self.rightmost

    def meets_position_policy(self, password: str):
        leftmost = password[self.leftmost - 1] == self.character
        rightmost = password[self.rightmost - 1] == self.character

        return rightmost ^ leftmost


with open("../input.txt", "r") as input:
    lines = input.read().splitlines()

pws_and_policies = []

for line in lines:
    policy, password = line.split(": ")

    pw_range, char = policy.split(" ")
    lower, upper = [int(x) for x in pw_range.split("-")]

    policy = PasswordPolicy(
        leftmost=lower,
        rightmost=upper,
        character=char
    )

    pws_and_policies.append((policy, password))

part_one = 0
part_two = 0

for policy, password in pws_and_policies:
    if policy.meets_repetition_policy(password):
        part_one += 1

    if policy.meets_position_policy(password):
        part_two += 1

print(f"Part 1: {part_one}")
print(f"Part 2: {part_two}")
