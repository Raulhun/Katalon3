import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import com.kms.katalon.core.webui.driver.DriverFactory as DriverFactory
import org.openqa.selenium.By as By
import org.openqa.selenium.WebElement as WebElement
import com.kms.katalon.core.annotation.Keyword as Keyword
import org.openqa.selenium.WebDriver as WebDriver
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil

WebUI.waitForElementVisible(findTestObject('PacienstorzsSearch/input_Keress_search'), 10)


WebDriver driver = DriverFactory.getWebDriver()

// Replace TableID with the Actual Table ID or Xpath reference
WebElement Webtable = driver.findElement(By.id('patients-table'))

//table[@id='patients-table']/tbody/tr[3]/td[2]
//Get the number of rows in the table and turn it into a List
List<WebElement> TotalRowCount = Webtable.findElements(By.xpath('//table[@id=\'patients-table\']/tbody/tr'))

//Display the number of rows in the table for the given sales rep
KeywordUtil.markWarning('No. of Rows in the WebTable: ' + TotalRowCount.size())

//Loop through the table and output the information to Log File
//Read columns 1-8, assign each to a variable, then output the result to the Log File

// (TotalRowCount.size() - 1)
int i = 0

table: for (int loop = 1; loop <= (TotalRowCount.size() - 1); loop++) {
    //println(loop)
	
    azonosito = driver.findElement(By.xpath(('//table[@id=\'patients-table\']/tbody/tr[' + (loop + 1)) + ']/td[4]')).getText()
    szulDat = driver.findElement(By.xpath(('//table[@id=\'patients-table\']/tbody/tr[' + (loop + 1)) + ']/td[3]')).getText()
    nev = driver.findElement(By.xpath(('//table[@id=\'patients-table\']/tbody/tr[' + (loop + 1)) + ']/td[2]')).getText()
	
	if(nev == GlobalVariable.patientNameFromData[loop-1] && szulDat == GlobalVariable.szuldatFromData[loop-1]){
		i++
		KeywordUtil.markWarning('A k?vetkez? p?ciens ' + i + ' alkalommal szerepel az adatb?zisban: ' + '_Teszt Alad?r')
	}
	table: break
}






