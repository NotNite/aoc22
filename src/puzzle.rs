pub trait Puzzle {
    fn test(&self) -> (String, String);
    fn one(&self, input: String) -> String;
    fn two(&self, input: String) -> String;
}
