import java.util.*;

public class JL_3_sumOfN {
  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    System.out.println("Please input the final number:");
    int input1 = sc.nextInt();
    int sum = 0;
    for (int i = 1; i <= input1; i++) {
      sum = sum + i;
    }
    System.out.println("The total = " + sum);
    sc.close();
  }
}
