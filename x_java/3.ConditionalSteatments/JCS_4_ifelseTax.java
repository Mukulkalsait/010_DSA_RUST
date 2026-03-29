
/**
 * JCS_4_ifelseTax
 */
import java.util.*;

public class JCS_4_ifelseTax {
  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    System.out.println("please Input your yearly Imcome ");
    int tax;
    int income = sc.nextInt();

    if (income < 500000) {
      tax = 0;
    } else if (income >= 500000 && income < 1000000) {
      tax = (int) (income * 0.2f);
    } else {
      tax = (int) (income * 0.3f);
    }
    int grossIncome = income - tax;
    System.out.println("accouding to your income '" + income + "' your income tax is = " + tax);
    System.out.println("hence your Real income is " + grossIncome);
    sc.close();
  }
}
