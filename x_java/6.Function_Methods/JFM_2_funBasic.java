import java.util.*;

public class JFM_2_funBasic {

  public static int multiply(int a, int b) {
    int multi = a * b;
    return multi;

  }

  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);

    System.out.println("Please input val1:");
    int a = sc.nextInt();

    System.out.println("Please input val2:");
    int b = sc.nextInt();

    int multi = multiply(a, b);
    System.out.println(multi);
    sc.close();
  }
}
