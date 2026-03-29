
//
//
//FIX:
// sum of all numbers from X to Y wiht FORMULA.

import java.util.*;

public class JCS_Assi4_2 {

  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    System.out.println("Inter Initial Value:");
    float initVal = sc.nextInt();
    System.out.println("Inter Final Value:");
    int finVal = sc.nextInt();

    float sum = (initVal + finVal) * (finVal - initVal) / 2;

    System.out.println(sum);
    sc.close();
  }
}
