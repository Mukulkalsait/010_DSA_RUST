import java.util.*;

public class JBD_2_evenOddSepretSum {

  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);

    int Even = 0;
    int Odd = 0;
    int repet = 1;
    int newNum = 0;
    do {

      System.out.println("Please enter Next Number:");
      newNum = sc.nextInt();
      if (newNum % 2 == 0) {
        Even = Even + newNum;
      } else {
        Odd = Odd + newNum;
      }

      System.out.println("Do you want to add more numbers (1 for Yes / 2 for NO):");
      repet = sc.nextInt();
    } while (repet == 1);

    System.out.println(" The total of all Evens is: " + Even + " and The total of all ODDS is: " + Odd);
    sc.close();
  }
}
