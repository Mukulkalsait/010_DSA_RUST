import java.util.*;

public class JL_1_while {
  public static void main(String[] args) {

    Scanner sc = new Scanner(System.in);
    System.out.println("Input loop Startar !!!");
    int a = sc.nextInt();

    int x = 1;
    while (x <= a) {
      System.out.println(x + " is the value of x");
      x++;
    }
    System.out.println("End.");
    sc.close();
  }
}
