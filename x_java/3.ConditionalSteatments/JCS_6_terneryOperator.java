
/**
 * JCS_6_terneryOperator
 */
import java.util.*;

public class JCS_6_terneryOperator {
  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    System.out.println("Insert Your Percentages:");
    int val1 = sc.nextInt();
    int val2 = 37;

    String remark = (val1 >= val2) ? "Congruats You are pass !!!" : "Your Score is low, Better Luck Next Time.";
    System.out.println(remark);

    // TODO:
    // ___________________In the above ternary operator we use
    // ( var) = (condition) ? True : False;
    // in case where VAR ( in any type) is given then ,,,, the true/false must
    // return only values for that var and not anyting else.
    //
    // FIX: in java ternory opeartor allwaya have 2 sides mean it will all ways get
    // STORED in a VAR so only messages can be return and like the BILLOW SETATEMENT
    // WILL NOT WORK>>>>>
    // (val1>= val2)?
    // System.out.println("congrats on geting "+ val1 " in your
    // exams.") :
    // System.out.println("you required " + (val2 - val1 ) + " more marks
    // to get pass the exam.");
    //
    // TODO: but : this can be done.

    System.out.println((val1 >= val2) ? "Congruats You are pass !!!" : "Your Score is low, Better Luck Next Time.");
    // here we added turnery opeartor insdide the println() function.
    sc.close();
  }
}
