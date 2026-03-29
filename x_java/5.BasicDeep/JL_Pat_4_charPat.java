import java.util.*;

public class JL_Pat_4_charPat {
  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);

    System.out.println("Inter line numbers:");
    int lines = sc.nextInt();
    int charValue = 1; // TODO: here we could easily used "char" and it will be worked.

    for (int i = 1; i < lines; i++) {
      for (int j = 1; j < i; j++) {
        char x = (char) (charValue); // WARN: if we usd "char" instead of "int" this could be easy.
        System.out.print(x);
        charValue++;
      }
      System.out.println();
    }
    sc.close();
  }
}
