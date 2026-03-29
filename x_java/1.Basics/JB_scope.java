
public class JB_scope {
  public static void main(String[] args) {

    // NOTE:
    // i. for loop variations:

    for (int i = 0, j = 1, k = 3, l = 21; i < args.length; i++) {
      System.out.println(i + "" + "" + "" + "" + j + "" + k + "" + l);
    }
    int x = j + k; // NOTE: we will not be able to access "j" & "k" outside of for loop's scope .

    // NOTE:
    // 2. noly scopen variations:
    //

    {
      int a = 12;
    }
    System.out.println(a); // NOTE: we will not be abel to access "a" here out of SCOPE ∴ =
  }
}
