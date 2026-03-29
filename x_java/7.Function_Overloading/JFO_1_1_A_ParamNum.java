import java.util.*;

public class JFO_1_1_A_ParamNum {

  // TODO:
  // type A different numbers of parameters:

  public static int multiply(int a, int b) {
    return a * b;
  }

  public static int multiply(int a, int b, int c) {
    return a * b * c;
  }

  public static int multiply(int a, int b, int c, int d) {
    return a * b * c * d;
  }

  public static int inputAdder() {
    Scanner sc = new Scanner(System.in);
    System.out.println("Intput a:");
    int a = sc.nextInt();
    System.out.println("Intput b:");
    int b = sc.nextInt();

    System.out.println("Do you want to add more? (y=1/n=2)");
    int y1 = sc.nextInt();

    if (y1 == 1) {
      System.out.println("Intput c:");
      int c = sc.nextInt();
      System.out.println("Do you want to add more? (y=1/n=2)");
      int y2 = sc.nextInt();
      if (y2 == 1) {
        System.out.println("Intput d: (max num = 4)");
        int d = sc.nextInt();
        int res = multiply(a, b, c, d);
        sc.close();
        return res;
      } else {
        int res = multiply(a, b, c);
        sc.close();
        return res;
      }
    } else {
      int res = multiply(a, b);
      sc.close();
      return res;
    }
  }

  public static void main(String[] args) {
    System.out.println(inputAdder());
  }
}
