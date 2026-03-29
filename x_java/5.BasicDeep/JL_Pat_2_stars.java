import java.util.*;

public class JL_Pat_2_stars {

  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);

    System.out.println("Please input number of lines you want to print:");
    int lines = sc.nextInt();
    int start = 1;
    while (lines >= start) { // FIX: we were using the int "start" for while and we might be going to put it
                             // inside for loop just in case.
      for (int i = 0; i < start; i++) { // TODO: becouse we want an int just for tihs loop so "for".
        System.out.print("*");
      }
      System.out.print("\n"); // TODO: to print in next line
      start++; // PERF: this must be in the while loop other wise it will be incrementino
               // everytime in for loop and loop will stop very soon.
    }
    sc.close();
  }
}
