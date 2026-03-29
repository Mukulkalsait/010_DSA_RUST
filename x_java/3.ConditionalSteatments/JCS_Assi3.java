
//
//
//
//
//FIX:
// Looop The Line N Times ... (While Loop)
//

import java.util.*;

public class JCS_Assi3 {

  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    System.out.println("Counter:");
    int a = sc.nextInt();
    int n = 1;
    while (n <= a) {
      System.out.println("This is the " + n + " th time Line LOOPED.");
      n++;
    }
    System.out.println("With this the loop is ended.");

    sc.close();
  }
}
