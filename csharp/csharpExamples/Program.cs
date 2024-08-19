using csharpExamples;

//initialize a list of Person objects
List<Person> people = new();

//create two Person objects and add them to the list
Person person1 = new Person
{
    Name = "John",
    Age = 25
};
people.Add(person1);

Person person2 = new Person
{
    Name = "Jane",
    Age = 22
};
people.Add(person2);

//iterate through the list and print out the details of each person
Console.WriteLine("List of people:");
foreach (Person person in people)
{
    Console.WriteLine($"Name: {person.Name}, Age: {person.Age}\n");
}